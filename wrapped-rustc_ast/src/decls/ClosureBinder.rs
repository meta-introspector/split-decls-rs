macro_rules! deps {
    () => {
        Closure!();
        Walkable!();
        GenericParam!();
    };
}

macro_rules! ClosureBinder {
    () => {
        deps!();
        # [doc = " Closure lifetime binder, `for<'a, 'b>` in `for<'a, 'b> |_: &'a (), _: &'b ()|`."] # [derive (Clone , Encodable , Decodable , Debug , Walkable)] pub enum ClosureBinder { # [doc = " The binder is not present, all closure lifetimes are inferred."] NotPresent , # [doc = " The binder is present."] For { # [doc = " Span of the whole `for<>` clause"] # [doc = ""] # [doc = " ```text"] # [doc = " for<'a, 'b> |_: &'a (), _: &'b ()| { ... }"] # [doc = " ^^^^^^^^^^^ -- this"] # [doc = " ```"] span : Span , # [doc = " Lifetimes in the `for<>` closure"] # [doc = ""] # [doc = " ```text"] # [doc = " for<'a, 'b> |_: &'a (), _: &'b ()| { ... }"] # [doc = "     ^^^^^^ -- this"] # [doc = " ```"] generic_params : ThinVec < GenericParam > , } , }
    };
}

ClosureBinder!();