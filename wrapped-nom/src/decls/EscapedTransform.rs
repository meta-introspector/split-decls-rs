macro_rules! deps {
    () => {
        Parser!();
    };
}

macro_rules! EscapedTransform {
    () => {
        deps!();
        # [doc = " Parser implementation for [escaped_transform]"] pub struct EscapedTransform < F , G , E , ExtendItem , Output > { normal : F , transform : G , control_char : char , e : PhantomData < E > , extend : PhantomData < ExtendItem > , o : PhantomData < Output > , }
    };
}

EscapedTransform!()