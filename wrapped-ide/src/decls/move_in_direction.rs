macro_rules! deps {
    () => {
        Direction!();
    };
}

macro_rules! move_in_direction {
    () => {
        deps!();
        fn move_in_direction (node : & SyntaxNode , direction : Direction , range : TextRange ,) -> Option < TextEdit > { match_ast ! { match node { ast :: ArgList (it) => swap_sibling_in_list (node , it . args () , range , direction) , ast :: GenericParamList (it) => swap_sibling_in_list (node , it . generic_params () , range , direction) , ast :: GenericArgList (it) => swap_sibling_in_list (node , it . generic_args () , range , direction) , ast :: VariantList (it) => swap_sibling_in_list (node , it . variants () , range , direction) , ast :: TypeBoundList (it) => swap_sibling_in_list (node , it . bounds () , range , direction) , _ => Some (replace_nodes (range , node , & match direction { Direction :: Up => node . prev_sibling () , Direction :: Down => node . next_sibling () , } ?)) } } }
    };
}

move_in_direction!()