// Generated macro for macro_532 (macro)
macro_rules! Depcrate_utilsmacro_532 {
() => {
// Module: crate::utils
// Provides: {"macro_532"}
// Dependencies: {}
pin_project ! { # [doc = " An `ActorFuture` that runs a function in the actor's context after a specified amount of time."] # [doc = ""] # [doc = " Unless you specifically need access to the future, use [`Context::run_later`] instead."] # [doc = ""] # [doc = " [`Context::run_later`]: ../prelude/trait.AsyncContext.html#method.run_later"] # [doc = ""] # [doc = " ```"] # [doc = " # use std::io;"] # [doc = " use std::time::Duration;"] # [doc = " use actix::prelude::*;"] # [doc = " use actix::utils::TimerFunc;"] # [doc = ""] # [doc = " struct MyActor;"] # [doc = ""] # [doc = " impl MyActor {"] # [doc = "     fn stop(&mut self, context: &mut Context<Self>) {"] # [doc = "         System::current().stop();"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " impl Actor for MyActor {"] # [doc = "    type Context = Context<Self>;"] # [doc = ""] # [doc = "    fn started(&mut self, context: &mut Context<Self>) {"] # [doc = "        // spawn a delayed future into our context"] # [doc = "        TimerFunc::new(Duration::from_millis(100), Self::stop)"] # [doc = "            .spawn(context);"] # [doc = "    }"] # [doc = " }"] # [doc = " # fn main() {"] # [doc = " #    let mut sys = System::new();"] # [doc = " #    let addr = sys.block_on(async { MyActor.start() });"] # [doc = " #    sys.run();"] # [doc = " # }"] # [must_use = "future do nothing unless polled"] # [allow (clippy :: type_complexity)] pub struct TimerFunc < A : Actor > { f : Option < Box < dyn FnOnce (& mut A , & mut A :: Context) >>, # [pin] timeout : Sleep , } }
};
}
