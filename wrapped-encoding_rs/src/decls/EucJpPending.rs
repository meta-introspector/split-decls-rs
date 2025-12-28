macro_rules! EucJpPending {
    () => {
        enum EucJpPending { None , Jis0208Lead (u8) , Jis0212Shift , Jis0212Lead (u8) , HalfWidthKatakana , }
    };
}

EucJpPending!()