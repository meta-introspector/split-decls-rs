macro_rules! deps {
    () => {
        AsyncAsSync!();
        Pending!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < 'r , 'ctx , T > AsyncAsSync < 'r , 'ctx , T > { # [doc = " Wraps an I/O handle implementing [`AsyncRead`] or [`AsyncWrite`] traits."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::AsyncAsSync;"] # [doc = " use std::task::Context;"] # [doc = " use waker_fn::waker_fn;"] # [doc = ""] # [doc = " let reader: &[u8] = b\"hello\";"] # [doc = " let waker = waker_fn(|| {});"] # [doc = " let mut context = Context::from_waker(&waker);"] # [doc = ""] # [doc = " let async_reader = AsyncAsSync::new(&mut context, reader);"] # [doc = " ```"] # [inline] pub fn new (context : & 'r mut Context < 'ctx > , inner : T) -> Self { AsyncAsSync { context , inner } } # [doc = " Attempt to shutdown the I/O handle."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::AsyncAsSync;"] # [doc = " use std::task::Context;"] # [doc = " use waker_fn::waker_fn;"] # [doc = ""] # [doc = " let reader: Vec<u8> = b\"hello\".to_vec();"] # [doc = " let waker = waker_fn(|| {});"] # [doc = " let mut context = Context::from_waker(&waker);"] # [doc = ""] # [doc = " let mut async_reader = AsyncAsSync::new(&mut context, reader);"] # [doc = " async_reader.close().unwrap();"] # [doc = " ```"] # [inline] pub fn close (& mut self) -> Result < () > where T : AsyncWrite + Unpin , { self . poll_with (| io , cx | io . poll_close (cx)) } # [doc = " Poll this `AsyncAsSync` for some function."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use futures_lite::io::{AsyncAsSync, AsyncRead};"] # [doc = " use std::task::Context;"] # [doc = " use waker_fn::waker_fn;"] # [doc = ""] # [doc = " let reader: &[u8] = b\"hello\";"] # [doc = " let waker = waker_fn(|| {});"] # [doc = " let mut context = Context::from_waker(&waker);"] # [doc = ""] # [doc = " let mut async_reader = AsyncAsSync::new(&mut context, reader);"] # [doc = " let r = async_reader.poll_with(|io, cx| io.poll_read(cx, &mut [0; 1024]));"] # [doc = " assert_eq!(r.unwrap(), 5);"] # [doc = " ```"] # [inline] pub fn poll_with < R > (& mut self , f : impl FnOnce (Pin < & mut T > , & mut Context < '_ >) -> Poll < Result < R > > ,) -> Result < R > where T : Unpin , { match f (Pin :: new (& mut self . inner) , self . context) { Poll :: Ready (res) => res , Poll :: Pending => Err (ErrorKind :: WouldBlock . into ()) , } } }
    };
}

impl_200!()