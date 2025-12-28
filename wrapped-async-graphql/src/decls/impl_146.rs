macro_rules! deps {
    () => {
        Cat!();
        Object!();
        FurColor!();
    };
}

macro_rules! impl_146 {
    () => {
        deps!();
        # [Object (internal)] impl Cat { async fn name (& self , surname : Option < bool >) -> Option < String > { unimplemented ! () } async fn nickname (& self) -> Option < String > { unimplemented ! () } async fn meows (& self) -> Option < bool > { unimplemented ! () } async fn meow_volume (& self) -> Option < i32 > { unimplemented ! () } async fn fur_color (& self) -> Option < FurColor > { unimplemented ! () } }
    };
}

impl_146!()