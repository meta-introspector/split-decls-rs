macro_rules! deps {
    () => {
        Figure!();
        Output!();
        Set!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl Set < Output > for Figure { # [doc = " Changes the output file"] # [doc = ""] # [doc = " **Note** The default output file is `output.plot`"] fn set (& mut self , output : Output) -> & mut Figure { self . output = output . 0 ; self } }
    };
}

impl_131!();