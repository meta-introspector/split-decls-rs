macro_rules! deps {
    () => {
        UnitHeader!();
        Reader!();
        Result!();
        DebuggingInformationEntry!();
        Error!();
        EntriesTree!();
        Abbreviations!();
        EntriesTreeNode!();
    };
}

macro_rules! impl_647 {
    () => {
        deps!();
        impl < 'abbrev , 'unit , R : Reader > EntriesTree < 'abbrev , 'unit , R > { fn new (root : R , unit : & 'unit UnitHeader < R > , abbreviations : & 'abbrev Abbreviations) -> Self { let input = root . clone () ; EntriesTree { root , unit , abbreviations , input , entry : None , depth : 0 , } } # [doc = " Returns the root node of the tree."] pub fn root < 'me > (& 'me mut self) -> Result < EntriesTreeNode < 'abbrev , 'unit , 'me , R > > { self . input = self . root . clone () ; self . entry = DebuggingInformationEntry :: parse (& mut self . input , self . unit , self . abbreviations) ? ; if self . entry . is_none () { return Err (Error :: UnexpectedNull) ; } self . depth = 0 ; Ok (EntriesTreeNode :: new (self , 1)) } # [doc = " Move the cursor to the next entry at the specified depth."] # [doc = ""] # [doc = " Requires `depth <= self.depth + 1`."] # [doc = ""] # [doc = " Returns `true` if successful."] fn next (& mut self , depth : isize) -> Result < bool > { if self . depth < depth { debug_assert_eq ! (self . depth + 1 , depth) ; match self . entry { Some (ref entry) => { if ! entry . has_children () { return Ok (false) ; } self . depth += 1 ; self . input = entry . after_attrs () ? ; } None => return Ok (false) , } if self . input . is_empty () { self . entry = None ; return Ok (false) ; } return match DebuggingInformationEntry :: parse (& mut self . input , self . unit , self . abbreviations ,) { Ok (entry) => { self . entry = entry ; Ok (self . entry . is_some ()) } Err (e) => { self . input . empty () ; self . entry = None ; Err (e) } } ; } loop { match self . entry { Some (ref entry) => { if entry . has_children () { if let Some (sibling_input) = entry . sibling () { self . input = sibling_input ; } else { self . depth += 1 ; self . input = entry . after_attrs () ? ; } } else { self . input = entry . after_attrs () ? ; } } None => { self . depth -= 1 ; } } if self . input . is_empty () { self . entry = None ; return Ok (false) ; } match DebuggingInformationEntry :: parse (& mut self . input , self . unit , self . abbreviations) { Ok (entry) => { self . entry = entry ; if self . depth == depth { return Ok (self . entry . is_some ()) ; } } Err (e) => { self . input . empty () ; self . entry = None ; return Err (e) ; } } } } }
    };
}

impl_647!();