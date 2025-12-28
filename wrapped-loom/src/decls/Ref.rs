macro_rules! Ref {
    () => {
        # [doc = " References an object in the store."] # [doc = ""] # [doc = " The reference tracks the type it references. Using `()` indicates the type"] # [doc = " is unknown."] # [derive (Eq , PartialEq)] # [cfg_attr (feature = "checkpoint" , derive (Serialize , Deserialize))] pub (super) struct Ref < T = () > { # [doc = " Index in the store"] index : usize , _p : PhantomData < T > , }
    };
}

Ref!();