macro_rules! OSControl {
    () => {
        # [derive (Eq , PartialEq , Debug)] enum OSControl < 'a , S : 'a + ToOwned + ? Sized > where < S as ToOwned > :: Owned : fmt :: Debug , { Title , Link { url : Cow < 'a , S > } , }
    };
}

OSControl!();