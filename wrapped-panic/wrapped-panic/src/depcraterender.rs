// Generated macro for render (function)
macro_rules! Depcraterender {
() => {
// Module: crate
// Provides: {"render"}
// Dependencies: {}
fn render (frame : & mut Frame , state : & PanicHandlerState) { let text = vec ! [Line :: from (format ! ("Panic hook is currently: {state:?}")) , Line :: from ("") , Line :: from ("Press `p` to cause a panic") , Line :: from ("Press `e` to cause an error") , Line :: from ("Press `h` to disable the panic hook") , Line :: from ("Press `q` to quit") , Line :: from ("") , Line :: from ("When your app panics without a panic hook, you will likely have to") , Line :: from ("reset your terminal afterwards with the `reset` command") , Line :: from ("") , Line :: from ("Try first with the panic handler enabled, and then with it disabled") , Line :: from ("to see the difference") ,] ; let paragraph = Paragraph :: new (text) . block (Block :: bordered () . title ("Panic Handler Demo")) . centered () ; frame . render_widget (paragraph , frame . area ()) ; }
};
}
