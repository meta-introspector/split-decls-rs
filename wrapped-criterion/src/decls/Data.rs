macro_rules! Data {
    () => {
        # [doc = " Bivariate `(X, Y)` data"] # [doc = ""] # [doc = " Invariants:"] # [doc = ""] # [doc = " - No `NaN`s in the data"] # [doc = " - At least two data points in the set"] pub struct Data < 'a , X , Y > (& 'a [X] , & 'a [Y]) ;
    };
}

Data!()