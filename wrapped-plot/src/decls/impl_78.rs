macro_rules! deps {
    () => {
        Matrix!();
        Axes!();
        Figure!();
        ErrorBarDefault!();
        Plot!();
        ErrorBar!();
        Data!();
        Properties!();
    };
}

macro_rules! impl_78 {
    () => {
        deps!();
        impl < X , Y , L , H > traits :: Plot < ErrorBar < X , Y , L , H > > for Figure where H : IntoIterator , H :: Item : Data , L : IntoIterator , L :: Item : Data , X : IntoIterator , X :: Item : Data , Y : IntoIterator , Y :: Item : Data , { type Properties = Properties ; fn plot < F > (& mut self , e : ErrorBar < X , Y , L , H > , configure : F) -> & mut Figure where F : FnOnce (& mut Properties) -> & mut Properties , { let (x_factor , y_factor) = crate :: scale_factor (& self . axes , crate :: Axes :: BottomXLeftY) ; let style = e . style () ; let (x , y , length , height , e_factor) = match e { ErrorBar :: XErrorBars { x , y , x_low , x_high , } | ErrorBar :: XErrorLines { x , y , x_low , x_high , } => (x , y , x_low , x_high , x_factor) , ErrorBar :: YErrorBars { x , y , y_low , y_high , } | ErrorBar :: YErrorLines { x , y , y_low , y_high , } => (x , y , y_low , y_high , y_factor) , } ; let data = Matrix :: new (itertools :: izip ! (x , y , length , height) , (x_factor , y_factor , e_factor , e_factor) ,) ; self . plots . push (Plot :: new (data , configure (& mut ErrorBarDefault :: default (style)) ,)) ; self } }
    };
}

impl_78!()