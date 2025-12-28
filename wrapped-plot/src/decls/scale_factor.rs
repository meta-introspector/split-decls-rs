macro_rules! deps {
    () => {
        Axis!();
        Axes!();
        ScaleFactorTrait!();
    };
}

macro_rules! scale_factor {
    () => {
        deps!();
        fn scale_factor (map : & map :: axis :: Map < axis :: Properties > , axes : Axes) -> (f64 , f64) { use crate :: Axes :: * ; use crate :: Axis :: * ; match axes { BottomXLeftY => (map . get (BottomX) . map_or (1. , ScaleFactorTrait :: scale_factor) , map . get (LeftY) . map_or (1. , ScaleFactorTrait :: scale_factor) ,) , BottomXRightY => (map . get (BottomX) . map_or (1. , ScaleFactorTrait :: scale_factor) , map . get (RightY) . map_or (1. , ScaleFactorTrait :: scale_factor) ,) , TopXLeftY => (map . get (TopX) . map_or (1. , ScaleFactorTrait :: scale_factor) , map . get (LeftY) . map_or (1. , ScaleFactorTrait :: scale_factor) ,) , TopXRightY => (map . get (TopX) . map_or (1. , ScaleFactorTrait :: scale_factor) , map . get (RightY) . map_or (1. , ScaleFactorTrait :: scale_factor) ,) , } }
    };
}

scale_factor!()