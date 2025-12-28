macro_rules! deps {
    () => {
        Scale!();
        Data!();
    };
}

macro_rules! Row {
    () => {
        deps!();
        # [doc = " Data that can serve as a row of the data matrix"] pub trait Row { # [doc = " Private"] type Scale : Copy ; # [doc = " Append this row to a buffer"] fn append_to (self , buffer : & mut Vec < u8 > , scale : Self :: Scale) ; # [doc = " Number of columns of the row"] fn ncols () -> usize ; }
    };
}

Row!()