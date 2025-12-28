macro_rules! deps {
    () => {
        Properties!();
    };
}

macro_rules! Configure {
    () => {
        deps!();
        # [doc = " Overloaded `configure` method"] pub trait Configure < This > { # [doc = " The properties of what's being configured"] type Properties ; # [doc = " Configure some set of properties"] fn configure < F > (& mut self , this : This , function : F) -> & mut Self where F : FnOnce (& mut Self :: Properties) -> & mut Self :: Properties ; }
    };
}

Configure!()