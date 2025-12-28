macro_rules! deps {
    () => {
        LabeledSample!();
        Float!();
        Iter!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl < 'a , A > LabeledSample < 'a , A > where A : Float , { # [doc = " Returns the number of data points per label"] # [doc = ""] # [doc = " - Time: `O(length)`"] # [allow (clippy :: similar_names)] pub fn count (& self) -> (usize , usize , usize , usize , usize) { let (mut los , mut lom , mut noa , mut him , mut his) = (0 , 0 , 0 , 0 , 0) ; for (_ , label) in self { match label { LowSevere => { los += 1 ; } LowMild => { lom += 1 ; } NotAnOutlier => { noa += 1 ; } HighMild => { him += 1 ; } HighSevere => { his += 1 ; } } } (los , lom , noa , him , his) } # [doc = " Returns the fences used to classify the outliers"] pub fn fences (& self) -> (A , A , A , A) { self . fences } # [doc = " Returns an iterator over the labeled data"] pub fn iter (& self) -> Iter < 'a , A > { Iter { fences : self . fences , iter : self . sample . iter () , } } }
    };
}

impl_357!();