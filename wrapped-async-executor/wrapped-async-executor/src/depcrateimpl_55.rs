// Generated macro for impl_55 (impl)
macro_rules! Depcrateimpl_55 {
() => {
// Module: crate
// Provides: {"impl_55"}
// Dependencies: {}
impl Sleepers { # [doc = " Inserts a new sleeping ticker."] fn insert (& mut self , waker : & Waker) -> usize { let id = match self . free_ids . pop () { Some (id) => id , None => self . count + 1 , } ; self . count += 1 ; self . wakers . push ((id , waker . clone ())) ; id } # [doc = " Re-inserts a sleeping ticker's waker if it was notified."] # [doc = ""] # [doc = " Returns `true` if the ticker was notified."] fn update (& mut self , id : usize , waker : & Waker) -> bool { for item in & mut self . wakers { if item . 0 == id { item . 1 . clone_from (waker) ; return false ; } } self . wakers . push ((id , waker . clone ())) ; true } # [doc = " Removes a previously inserted sleeping ticker."] # [doc = ""] # [doc = " Returns `true` if the ticker was notified."] fn remove (& mut self , id : usize) -> bool { self . count -= 1 ; self . free_ids . push (id) ; for i in (0 .. self . wakers . len ()) . rev () { if self . wakers [i] . 0 == id { self . wakers . remove (i) ; return false ; } } true } # [doc = " Returns `true` if a sleeping ticker is notified or no tickers are sleeping."] fn is_notified (& self) -> bool { self . count == 0 || self . count > self . wakers . len () } # [doc = " Returns notification waker for a sleeping ticker."] # [doc = ""] # [doc = " If a ticker was notified already or there are no tickers, `None` will be returned."] fn notify (& mut self) -> Option < Waker > { if self . wakers . len () == self . count { self . wakers . pop () . map (| item | item . 1) } else { None } } }
};
}
