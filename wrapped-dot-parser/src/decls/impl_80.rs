macro_rules! deps {
    () => {
        CompassPt!();
    };
}

macro_rules! impl_80 {
    () => {
        deps!();
        # [cfg (feature = "display")] impl Display for CompassPt { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , std :: fmt :: Error > { match self { CompassPt :: N => write ! (f , "n") , CompassPt :: NE => write ! (f , "ne") , CompassPt :: E => write ! (f , "e") , CompassPt :: SE => write ! (f , "se") , CompassPt :: S => write ! (f , "s") , CompassPt :: SW => write ! (f , "sw") , CompassPt :: W => write ! (f , "w") , CompassPt :: NW => write ! (f , "nw") , CompassPt :: C => write ! (f , "c") , CompassPt :: Underscore => write ! (f , "_") , } } }
    };
}

impl_80!()