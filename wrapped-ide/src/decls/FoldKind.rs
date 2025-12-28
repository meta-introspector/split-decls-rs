macro_rules! FoldKind {
    () => {
        # [derive (Debug , PartialEq , Eq)] pub enum FoldKind { Comment , Imports , Region , Block , ArgList , Array , WhereClause , ReturnType , MatchArm , Function , Modules , Consts , Statics , TypeAliases , ExternCrates , }
    };
}

FoldKind!();