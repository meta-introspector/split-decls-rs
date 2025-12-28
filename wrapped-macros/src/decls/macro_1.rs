macro_rules! macro_1 {
    () => {
        decl_derive ! ([TypeVisitable , attributes (type_visitable)] => # [doc = " Derives `TypeVisitable` for the annotated `struct` or `enum` (`union` is not supported)."] # [doc = ""] # [doc = " Each field of the struct or enum variant will be visited in definition order, using the"] # [doc = " `TypeVisitable` implementation for its type. However, if a field of a struct or an enum"] # [doc = " variant is annotated with `#[type_visitable(ignore)]` then that field will not be"] # [doc = " visited (and its type is not required to implement `TypeVisitable`)."] type_visitable_derive) ;
    };
}

macro_1!();