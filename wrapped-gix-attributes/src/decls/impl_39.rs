macro_rules! deps {
    () => {
        MetadataCollection!();
        Assignments!();
        Attributes!();
        Value!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        # [doc = " Mutation"] impl MetadataCollection { # [doc = " Assign order ids to each attribute either in macros (along with macros themselves) or attributes of patterns, and store"] # [doc = " them in this collection."] # [doc = ""] # [doc = " Must be called before querying matches."] pub fn update_from_list (& mut self , list : & mut gix_glob :: search :: pattern :: List < Attributes >) { for pattern in & mut list . patterns { match & mut pattern . value { Value :: MacroAssignments { id : order , assignments } => { * order = self . id_for_macro (pattern . pattern . text . to_str () . expect ("valid macro names are always UTF8 and this was verified") , assignments ,) ; } Value :: Assignments (assignments) => { self . assign_order_to_attributes (assignments) ; } } } } }
    };
}

impl_39!()