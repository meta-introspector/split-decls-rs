macro_rules! deps {
    () => {
        LzmaCoder!();
    };
}

macro_rules! impl_307 {
    () => {
        deps!();
        impl LzmaCoder { pub (crate) fn new (pb : usize) -> Self { let mut c = Self { pos_mask : (1 << pb) - 1 , reps : Default :: default () , state : Default :: default () , is_match : Default :: default () , is_rep : Default :: default () , is_rep0 : Default :: default () , is_rep1 : Default :: default () , is_rep2 : Default :: default () , is_rep0_long : Default :: default () , dist_slots : [[Default :: default () ; DIST_SLOTS] ; DIST_STATES] , dist_special : [Default :: default () ; 124] , dist_align : Default :: default () , } ; c . reset () ; c } pub (crate) fn reset (& mut self) { self . reps = [0 ; REPS] ; self . state . reset () ; for ele in self . is_match . iter_mut () { init_probs (ele) ; } init_probs (& mut self . is_rep) ; init_probs (& mut self . is_rep0) ; init_probs (& mut self . is_rep1) ; init_probs (& mut self . is_rep2) ; for ele in self . is_rep0_long . iter_mut () { init_probs (ele) ; } for ele in self . dist_slots . iter_mut () { init_probs (ele) ; } init_probs (& mut self . dist_special) ; init_probs (& mut self . dist_align) ; } # [inline (always)] pub (crate) fn get_dist_special (& mut self , i : usize) -> & mut [u16] { & mut self . dist_special [DIST_SPECIAL_INDEX [i] .. DIST_SPECIAL_END [i]] } }
    };
}

impl_307!()