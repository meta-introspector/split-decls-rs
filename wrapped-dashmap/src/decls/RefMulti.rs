macro_rules! RefMulti {
    () => {
        pub struct RefMulti < 'a , K > { inner : mapref :: multiple :: RefMulti < 'a , K , () > , }
    };
}

RefMulti!();