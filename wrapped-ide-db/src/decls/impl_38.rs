macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_38 {
    () => {
        deps!();
        impl AsAssocItem for Definition { fn as_assoc_item (self , db : & dyn hir :: db :: HirDatabase) -> Option < AssocItem > { match self { Definition :: Function (it) => it . as_assoc_item (db) , Definition :: Const (it) => it . as_assoc_item (db) , Definition :: TypeAlias (it) => it . as_assoc_item (db) , _ => None , } } }
    };
}

impl_38!();