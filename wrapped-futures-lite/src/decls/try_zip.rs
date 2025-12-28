macro_rules! try_zip {
    () => {
        # [doc = " Joins two fallible futures, waiting for both to complete or one of them to error."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::future;"] # [doc = ""] # [doc = " # spin_on::spin_on(async {"] # [doc = " let a = async { Ok::<i32, i32>(1) };"] # [doc = " let b = async { Err::<i32, i32>(2) };"] # [doc = ""] # [doc = " assert_eq!(future::try_zip(a, b).await, Err(2));"] # [doc = " # })"] # [doc = " ```"] pub fn try_zip < T1 , T2 , E , F1 , F2 > (future1 : F1 , future2 : F2) -> TryZip < F1 , T1 , F2 , T2 > where F1 : Future < Output = Result < T1 , E > > , F2 : Future < Output = Result < T2 , E > > , { TryZip { future1 : Some (future1) , future2 : Some (future2) , output1 : None , output2 : None , } }
    };
}

try_zip!()