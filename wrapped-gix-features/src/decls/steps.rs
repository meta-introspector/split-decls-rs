macro_rules! steps {
    () => {
        # [doc = " A predefined unit for displaying a multi-step progress"] pub fn steps () -> Option < Unit > { Some (unit :: dynamic (unit :: Range :: new ("steps"))) }
    };
}

steps!()