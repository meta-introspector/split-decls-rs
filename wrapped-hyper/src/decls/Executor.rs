macro_rules! Executor {
    () => {
        # [doc = " An executor of futures."] # [doc = ""] # [doc = " This trait allows Hyper to abstract over async runtimes. Implement this trait for your own type."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # use hyper::rt::Executor;"] # [doc = " # use std::future::Future;"] # [doc = " #[derive(Clone)]"] # [doc = " struct TokioExecutor;"] # [doc = ""] # [doc = " impl<F> Executor<F> for TokioExecutor"] # [doc = " where"] # [doc = "     F: Future + Send + 'static,"] # [doc = "     F::Output: Send + 'static,"] # [doc = " {"] # [doc = "     fn execute(&self, future: F) {"] # [doc = "         tokio::spawn(future);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] pub trait Executor < Fut > { # [doc = " Place the future into the executor to be run."] fn execute (& self , fut : Fut) ; }
    };
}

Executor!()