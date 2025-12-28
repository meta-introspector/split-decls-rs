macro_rules! deps {
    () => {
        Figure!();
        Matrix!();
        Plot!();
        Properties!();
        CurveDefault!();
        Axes!();
        Data!();
        Curve!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl < X , Y > traits :: Plot < Curve < X , Y > > for Figure where X : IntoIterator , X :: Item : Data , Y : IntoIterator , Y :: Item : Data , { type Properties = Properties ; fn plot < F > (& mut self , curve : Curve < X , Y > , configure : F) -> & mut Figure where F : FnOnce (& mut Properties) -> & mut Properties , { let style = curve . style () ; let (x , y) = match curve { Curve :: Dots { x , y } | Curve :: Impulses { x , y } | Curve :: Lines { x , y } | Curve :: LinesPoints { x , y } | Curve :: Points { x , y } | Curve :: Steps { x , y } => (x , y) , } ; let mut props = CurveDefault :: default (style) ; configure (& mut props) ; let (x_factor , y_factor) = crate :: scale_factor (& self . axes , props . axes . unwrap_or (crate :: Axes :: BottomXLeftY)) ; let data = Matrix :: new (itertools :: izip ! (x , y) , (x_factor , y_factor)) ; self . plots . push (Plot :: new (data , & props)) ; self } }
    };
}

impl_63!()