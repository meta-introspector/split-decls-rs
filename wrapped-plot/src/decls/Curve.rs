macro_rules! Curve {
    () => {
        # [doc = " Types of \"curve\" plots"] pub enum Curve < X , Y > { # [doc = " A minimally sized dot on each data point"] Dots { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , } , # [doc = " A vertical \"impulse\" on each data point"] Impulses { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , } , # [doc = " Line that joins the data points"] Lines { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , } , # [doc = " Line with a point on each data point"] LinesPoints { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , } , # [doc = " A point on each data point"] Points { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , } , # [doc = " An step `_|` between each data point"] Steps { # [doc = " X coordinate of the data points"] x : X , # [doc = " Y coordinate of the data points"] y : Y , } , }
    };
}

Curve!();