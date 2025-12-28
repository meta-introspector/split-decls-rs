macro_rules! deps {
    () => {
        DocumentOperations!();
        Error!();
        Positioned!();
        ExecutableDocument!();
        Result!();
        DefinitionItem!();
        GraphQLParser!();
        Rule!();
        FragmentDefinition!();
        PositionCalculator!();
    };
}

macro_rules! parse_query {
    () => {
        deps!();
        # [doc = " Parse a GraphQL query document."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Fails if the query is not a valid GraphQL document."] pub fn parse_query < T : AsRef < str > > (input : T) -> Result < ExecutableDocument > { let mut pc = PositionCalculator :: new (input . as_ref ()) ; let pairs = GraphQLParser :: parse (Rule :: executable_document , input . as_ref ()) ? ; let items = parse_definition_items (exactly_one (pairs) , & mut pc) ? ; let mut operations = None ; let mut fragments : HashMap < _ , Positioned < FragmentDefinition > > = HashMap :: new () ; for item in items { match item { DefinitionItem :: Operation (item) => { if let Some (name) = item . node . name { let operations = operations . get_or_insert_with (| | DocumentOperations :: Multiple (HashMap :: new ())) ; let operations = match operations { DocumentOperations :: Single (anonymous) => { return Err (Error :: MultipleOperations { anonymous : anonymous . pos , operation : item . pos , }) ; } DocumentOperations :: Multiple (operations) => operations , } ; match operations . entry (name . node) { hash_map :: Entry :: Occupied (entry) => { let (name , first) = entry . remove_entry () ; return Err (Error :: OperationDuplicated { operation : name , first : first . pos , second : item . pos , }) ; } hash_map :: Entry :: Vacant (entry) => { entry . insert (Positioned :: new (item . node . definition , item . pos)) ; } } } else { match operations { Some (operations) => { return Err (Error :: MultipleOperations { anonymous : item . pos , operation : match operations { DocumentOperations :: Single (single) => single . pos , DocumentOperations :: Multiple (map) => { map . values () . next () . unwrap () . pos } } , }) ; } None => { operations = Some (DocumentOperations :: Single (Positioned :: new (item . node . definition , item . pos ,))) ; } } } } DefinitionItem :: Fragment (item) => match fragments . entry (item . node . name . node) { hash_map :: Entry :: Occupied (entry) => { let (name , first) = entry . remove_entry () ; return Err (Error :: FragmentDuplicated { fragment : name , first : first . pos , second : item . pos , }) ; } hash_map :: Entry :: Vacant (entry) => { entry . insert (Positioned :: new (item . node . definition , item . pos)) ; } } , } } Ok (ExecutableDocument { operations : operations . ok_or (Error :: MissingOperation) ? , fragments , }) }
    };
}

parse_query!();