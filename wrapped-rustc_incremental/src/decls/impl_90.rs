macro_rules! deps {
    () => {
        FindAllAttrs!();
        UncheckedClean!();
    };
}

macro_rules! impl_90 {
    () => {
        deps!();
        impl < 'tcx > FindAllAttrs < 'tcx > { fn is_active_attr (& mut self , attr : & Attribute) -> bool { if attr . has_name (sym :: rustc_clean) && check_config (self . tcx , attr) { return true ; } false } fn report_unchecked_attrs (& self , mut checked_attrs : FxHashSet < ast :: AttrId >) { for attr in & self . found_attrs { if ! checked_attrs . contains (& attr . id ()) { self . tcx . dcx () . emit_err (errors :: UncheckedClean { span : attr . span () }) ; checked_attrs . insert (attr . id ()) ; } } } }
    };
}

impl_90!();