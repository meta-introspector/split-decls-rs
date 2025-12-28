macro_rules! deps {
    () => {
        EnumDef!();
        Visitor!();
    };
}

macro_rules! walk_enum_def {
    () => {
        deps!();
        pub fn walk_enum_def < 'v , V : Visitor < 'v > > (visitor : & mut V , enum_definition : & 'v EnumDef < 'v > ,) -> V :: Result { let EnumDef { variants } = enum_definition ; walk_list ! (visitor , visit_variant , * variants) ; V :: Result :: output () }
    };
}

walk_enum_def!();