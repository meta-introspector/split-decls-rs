macro_rules! deps {
    () => {
        Vertical!();
        Horizontal!();
    };
}

macro_rules! ErrorBar {
    () => {
        deps!();
        # [doc = " Asymmetric error bar plots"] pub enum ErrorBar < X , Y , L , H > { # [doc = " Horizontal error bars"] XErrorBars { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , # [doc = " X coordinate of the left end of the error bar"] x_low : L , # [doc = " Y coordinate of the right end of the error bar"] x_high : H , } , # [doc = " Horizontal error bars, where each point is joined by a line"] XErrorLines { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , # [doc = " X coordinate of the left end of the error bar"] x_low : L , # [doc = " Y coordinate of the right end of the error bar"] x_high : H , } , # [doc = " Vertical error bars"] YErrorBars { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , # [doc = " Y coordinate of the bottom of the error bar"] y_low : L , # [doc = " Y coordinate of the top of the error bar"] y_high : H , } , # [doc = " Vertical error bars, where each point is joined by a line"] YErrorLines { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , # [doc = " Y coordinate of the bottom of the error bar"] y_low : L , # [doc = " Y coordinate of the top of the error bar"] y_high : H , } , }
    };
}

ErrorBar!()