macro_rules! impl_drop_trait {
    () => {
        # [doc = " Macro that implements the `Drop` trait on a object called `$name` which has"] # [doc = " a field `value`. This `Drop` will zero out the field `value` when the"] # [doc = " objects destructor is called."] macro_rules ! impl_drop_trait (($ name : ident) => (impl Drop for $ name { fn drop (& mut self) { use zeroize :: Zeroize ; self . value . iter_mut () . zeroize () ; } })) ;
    };
}

impl_drop_trait!()