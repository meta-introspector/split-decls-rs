macro_rules! serialize_trait_object {
    () => {
        # [doc = " Implement `serde::Serialize` for a trait object that has"] # [doc = " `erased_serde::Serialize` as a supertrait."] # [doc = ""] # [doc = " ```"] # [doc = " use erased_serde::serialize_trait_object;"] # [doc = ""] # [doc = " trait Event: erased_serde::Serialize {"] # [doc = "     /* ... */"] # [doc = " }"] # [doc = ""] # [doc = " erased_serde::serialize_trait_object!(Event);"] # [doc = " ```"] # [doc = ""] # [doc = " The macro supports traits that have type parameters and/or `where` clauses."] # [doc = ""] # [doc = " ```"] # [doc = " # use erased_serde::serialize_trait_object;"] # [doc = " #"] # [doc = " trait Difficult<T>: erased_serde::Serialize where T: Copy {"] # [doc = "     /* ... */"] # [doc = " }"] # [doc = ""] # [doc = " serialize_trait_object!(<T> Difficult<T> where T: Copy);"] # [doc = " ```"] # [macro_export] macro_rules ! serialize_trait_object { ($ ($ path : tt) +) => { $ crate :: __internal_serialize_trait_object ! (begin $ ($ path) +) ; } ; }
    };
}

serialize_trait_object!();