macro_rules! deps {
    () => {
        DogCommand!();
        Dog!();
        Object!();
    };
}

macro_rules! impl_143 {
    () => {
        deps!();
        # [Object (internal)] impl Dog { async fn name (& self , surname : Option < bool >) -> Option < String > { unimplemented ! () } async fn nickname (& self) -> Option < String > { unimplemented ! () } async fn bark_volume (& self) -> Option < i32 > { unimplemented ! () } async fn barks (& self) -> Option < bool > { unimplemented ! () } async fn does_know_command (& self , dog_command : Option < DogCommand >) -> Option < bool > { unimplemented ! () } async fn is_housetrained (& self , # [graphql (default = true)] at_other_homes : bool ,) -> Option < bool > { unimplemented ! () } async fn is_at_location (& self , x : Option < i32 > , y : Option < i32 >) -> Option < bool > { unimplemented ! () } }
    };
}

impl_143!()