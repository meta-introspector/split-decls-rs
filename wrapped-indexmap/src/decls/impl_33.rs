macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < K : Value , V : Value , S > Value for IndexMap < K , V , S > { fn stream < 'sval , ST : Stream < 'sval > + ? Sized > (& 'sval self , stream : & mut ST) -> sval :: Result { stream . map_begin (Some (self . len ())) ? ; for (k , v) in self { stream . map_key_begin () ? ; stream . value (k) ? ; stream . map_key_end () ? ; stream . map_value_begin () ? ; stream . value (v) ? ; stream . map_value_end () ? ; } stream . map_end () } }
    };
}

impl_33!();