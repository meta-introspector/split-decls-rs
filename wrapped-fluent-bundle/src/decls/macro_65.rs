macro_rules! deps {
    () => {
        Resource!();
    };
}

macro_rules! macro_65 {
    () => {
        deps!();
        self_cell ! (pub struct InnerFluentResource { owner : String , # [covariant] dependent : Resource , } impl { Debug }) ;
    };
}

macro_65!();