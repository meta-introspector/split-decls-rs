macro_rules! deps {
    () => {
        StepBy!();
        Producer!();
        IntoIter!();
        StepByProducer!();
    };
}

macro_rules! impl_858 {
    () => {
        deps!();
        impl < P > Producer for StepByProducer < P > where P : Producer , { type Item = P :: Item ; type IntoIter = iter :: StepBy < P :: IntoIter > ; fn into_iter (self) -> Self :: IntoIter { self . base . into_iter () . step_by (self . step) } fn split_at (self , index : usize) -> (Self , Self) { let elem_index = Ord :: min (index * self . step , self . len) ; let (left , right) = self . base . split_at (elem_index) ; (StepByProducer { base : left , step : self . step , len : elem_index , } , StepByProducer { base : right , step : self . step , len : self . len - elem_index , } ,) } fn min_len (& self) -> usize { self . base . min_len () . div_ceil (self . step) } fn max_len (& self) -> usize { self . base . max_len () / self . step } }
    };
}

impl_858!()