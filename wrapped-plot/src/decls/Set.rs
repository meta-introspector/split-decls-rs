macro_rules! Set {
    () => {
        # [doc = " Overloaded `set` method"] pub trait Set < T > { # [doc = " Sets some property"] fn set (& mut self , value : T) -> & mut Self ; }
    };
}

Set!()