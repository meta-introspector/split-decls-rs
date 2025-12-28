macro_rules! deps {
    () => {
        EntryRef!();
        Outcome!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < 'a > EntryRef < 'a > { pub (super) fn from_outcome (rela_path : Cow < 'a , BStr > , info : crate :: walk :: classify :: Outcome) -> Self { EntryRef { rela_path , property : info . property , status : info . status , disk_kind : info . disk_kind , index_kind : info . index_kind , pathspec_match : info . pathspec_match , } } }
    };
}

impl_29!()