macro_rules! deps {
    () => {
        LabeledSample!();
        Float!();
        Label!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl < 'a , A > Index < usize > for LabeledSample < 'a , A > where A : Float , { type Output = Label ; # [allow (clippy :: similar_names)] fn index (& self , i : usize) -> & Label { static LOW_SEVERE : Label = LowSevere ; static LOW_MILD : Label = LowMild ; static HIGH_MILD : Label = HighMild ; static HIGH_SEVERE : Label = HighSevere ; static NOT_AN_OUTLIER : Label = NotAnOutlier ; let x = self . sample [i] ; let (lost , lomt , himt , hist) = self . fences ; if x < lost { & LOW_SEVERE } else if x > hist { & HIGH_SEVERE } else if x < lomt { & LOW_MILD } else if x > himt { & HIGH_MILD } else { & NOT_AN_OUTLIER } } }
    };
}

impl_359!();