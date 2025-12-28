macro_rules! zip {
    () => {
        # [doc = " Joins two futures, waiting for both to complete."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let a = async { 1 };"] # [doc = " let b = async { 2 };"] # [doc = ""] # [doc = " assert_eq!(future::zip(a, b).await, (1, 2));"] # [doc = " # })"] # [doc = " ```"] pub fn zip < F1 , F2 > (future1 : F1 , future2 : F2) -> Zip < F1 , F2 > where F1 : Future , F2 : Future , { Zip { future1 : Some (future1) , future2 : Some (future2) , output1 : None , output2 : None , } }
    };
}

zip!()