macro_rules! deps {
    () => {
        Position!();
        Stacked!();
        Justification!();
        Order!();
    };
}

macro_rules! Properties {
    () => {
        deps!();
        # [doc = " Properties of the key"] # [derive (Clone)] pub struct Properties { boxed : bool , hidden : bool , justification : Option < Justification > , order : Option < Order > , position : Option < Position > , stacked : Option < Stacked > , title : Option < Cow < 'static , str > > , }
    };
}

Properties!()