macro_rules! deps {
    () => {
        LocalFieldId!();
        DefDatabase!();
        VariantId!();
    };
}

macro_rules! fields_attrs_source_map {
    () => {
        deps!();
        pub (crate) fn fields_attrs_source_map (db : & dyn DefDatabase , def : VariantId ,) -> Arc < ArenaMap < LocalFieldId , AstPtr < Either < ast :: TupleField , ast :: RecordField > > > > { let mut res = ArenaMap :: default () ; let child_source = def . child_source (db) ; for (idx , variant) in child_source . value . iter () { res . insert (idx , variant . as_ref () . either (| l | AstPtr :: new (l) . wrap_left () , | r | AstPtr :: new (r) . wrap_right ()) ,) ; } Arc :: new (res) }
    };
}

fields_attrs_source_map!();