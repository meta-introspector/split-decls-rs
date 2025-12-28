macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl AsExternAssocItem for Definition { fn as_extern_assoc_item (self , db : & dyn hir :: db :: HirDatabase) -> Option < ExternAssocItem > { match self { Definition :: Function (it) => it . as_extern_assoc_item (db) , Definition :: Static (it) => it . as_extern_assoc_item (db) , Definition :: TypeAlias (it) => it . as_extern_assoc_item (db) , _ => None , } } }
    };
}

impl_39!();