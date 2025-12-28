macro_rules! deps {
    () => {
        Log!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl Log { fn maybe_log (& self) { if self . current_level > self . max_level { return ; } let step = self . step () ; if self . trigger . swap (false , Ordering :: Relaxed) { match (self . max , & self . unit) { (max , Some (unit)) => log :: info ! ("{} → {}" , self . name , unit . display (step , max , None)) , (Some (max) , None) => log :: info ! ("{} → {} / {}" , self . name , step , max) , (None , None) => log :: info ! ("{} → {}" , self . name , step) , } } } }
    };
}

impl_180!()