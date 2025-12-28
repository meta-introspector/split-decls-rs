macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_364 {
    () => {
        deps!();
        impl Label { # [doc = " Checks if the data point has an \"unusually\" high value"] pub fn is_high (& self) -> bool { matches ! (* self , HighMild | HighSevere) } # [doc = " Checks if the data point is labeled as a \"mild\" outlier"] pub fn is_mild (& self) -> bool { matches ! (* self , HighMild | LowMild) } # [doc = " Checks if the data point has an \"unusually\" low value"] pub fn is_low (& self) -> bool { matches ! (* self , LowMild | LowSevere) } # [doc = " Checks if the data point is labeled as an outlier"] pub fn is_outlier (& self) -> bool { ! matches ! (* self , NotAnOutlier) } # [doc = " Checks if the data point is labeled as a \"severe\" outlier"] pub fn is_severe (& self) -> bool { matches ! (* self , HighSevere | LowSevere) } }
    };
}

impl_364!();