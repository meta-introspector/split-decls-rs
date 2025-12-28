macro_rules! deps {
    () => {
        FormatWith!();
    };
}

macro_rules! new_format {
    () => {
        deps!();
        pub fn new_format < I , F > (iter : I , separator : & str , f : F) -> FormatWith < '_ , I , F > where I : Iterator , F : FnMut (I :: Item , & mut dyn FnMut (& dyn fmt :: Display) -> fmt :: Result) -> fmt :: Result , { FormatWith { sep : separator , inner : Cell :: new (Some ((iter , f))) , } }
    };
}

new_format!()