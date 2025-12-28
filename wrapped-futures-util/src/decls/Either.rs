macro_rules! Either {
    () => {
        # [doc = " Combines two different futures, streams, or sinks having the same associated types into a single type."] # [doc = ""] # [doc = " This is useful when conditionally choosing between two distinct future types:"] # [doc = ""] # [doc = " ```rust"] # [doc = " use futures::future::Either;"] # [doc = ""] # [doc = " # futures::executor::block_on(async {"] # [doc = " let cond = true;"] # [doc = ""] # [doc = " let fut = if cond {"] # [doc = "     Either::Left(async move { 12 })"] # [doc = " } else {"] # [doc = "     Either::Right(async move { 44 })"] # [doc = " };"] # [doc = ""] # [doc = " assert_eq!(fut.await, 12);"] # [doc = " # })"] # [doc = " ```"] # [derive (Debug , Clone)] pub enum Either < A , B > { # [doc = " First branch of the type"] Left (A) , # [doc = " Second branch of the type"] Right (B) , }
    };
}

Either!();