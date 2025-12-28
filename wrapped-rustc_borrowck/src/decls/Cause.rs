macro_rules! Cause {
    () => {
        # [doc = " N.B., the variants in `Cause` are intentionally ordered. Lower"] # [doc = " values are preferred when it comes to error messages. Do not"] # [doc = " reorder willy nilly."] # [derive (Copy , Clone , Debug , PartialOrd , Ord , PartialEq , Eq)] pub (crate) enum Cause { # [doc = " point inserted because Local was live at the given Location"] LiveVar (Local , Location) , # [doc = " point inserted because Local was dropped at the given Location"] DropVar (Local , Location) , }
    };
}

Cause!();