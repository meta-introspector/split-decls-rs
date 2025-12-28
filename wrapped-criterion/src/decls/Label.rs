macro_rules! Label {
    () => {
        # [doc = " Labels used to classify outliers"] pub enum Label { # [doc = " A \"mild\" outlier in the \"high\" spectrum"] HighMild , # [doc = " A \"severe\" outlier in the \"high\" spectrum"] HighSevere , # [doc = " A \"mild\" outlier in the \"low\" spectrum"] LowMild , # [doc = " A \"severe\" outlier in the \"low\" spectrum"] LowSevere , # [doc = " A normal data point"] NotAnOutlier , }
    };
}

Label!()