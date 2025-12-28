macro_rules! deps {
    () => {
        Figure!();
    };
}

macro_rules! Title {
    () => {
        deps!();
        # [doc = " Figure title"] pub struct Title (Cow < 'static , str >) ;
    };
}

Title!()