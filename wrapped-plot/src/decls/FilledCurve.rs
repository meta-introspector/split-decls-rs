macro_rules! FilledCurve {
    () => {
        # [doc = " Fills the area between two curves"] pub struct FilledCurve < X , Y1 , Y2 > { # [doc = " X coordinate of the data points of both curves"] pub x : X , # [doc = " Y coordinate of the data points of the first curve"] pub y1 : Y1 , # [doc = " Y coordinate of the data points of the second curve"] pub y2 : Y2 , }
    };
}

FilledCurve!()