macro_rules! deps {
    () => {
        RpitContext!();
    };
}

macro_rules! OpaqueTyOrigin {
    () => {
        deps!();
        # [doc = " From whence the opaque type came."] # [derive (Copy , Clone , PartialEq , Eq , Debug)] # [derive (HashStable_Generic , Encodable , Decodable)] pub enum OpaqueTyOrigin < D > { # [doc = " `-> impl Trait`"] FnReturn { # [doc = " The defining function."] parent : D , in_trait_or_impl : Option < RpitContext > , } , # [doc = " `async fn`"] AsyncFn { # [doc = " The defining function."] parent : D , in_trait_or_impl : Option < RpitContext > , } , # [doc = " type aliases: `type Foo = impl Trait;`"] TyAlias { # [doc = " The type alias or associated type parent of the TAIT/ATPIT"] parent : D , # [doc = " associated types in impl blocks for traits."] in_assoc_ty : bool , } , }
    };
}

OpaqueTyOrigin!()