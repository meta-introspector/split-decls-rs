macro_rules! deps {
    () => {
        BoxFieldFuture!();
    };
}

macro_rules! Fields {
    () => {
        deps!();
        # [doc = " A set of fields on an container that are being selected."] pub struct Fields < 'a > (Vec < BoxFieldFuture < 'a > >) ;
    };
}

Fields!()