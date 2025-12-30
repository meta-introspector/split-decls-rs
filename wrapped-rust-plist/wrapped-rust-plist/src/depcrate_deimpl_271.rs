// Generated macro for impl_271 (impl)
macro_rules! Depcrate_deimpl_271 {
() => {
// Module: crate::de
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'event , I > Deserializer < 'event , I > where I : IntoIterator < Item = Result < Event < 'event > , Error > > , { pub fn new (iter : I) -> Deserializer < 'event , I > { Deserializer { events : iter . into_iter () . peekable () , option_mode : OptionMode :: Root , in_plist_value : false , } } fn with_option_mode < T , F : FnOnce (& mut Deserializer < 'event , I >) -> Result < T , Error > > (& mut self , option_mode : OptionMode , f : F ,) -> Result < T , Error > { let prev_option_mode = mem :: replace (& mut self . option_mode , option_mode) ; let ret = f (& mut * self) ; self . option_mode = prev_option_mode ; ret } fn enter_plist_value < T , F : FnOnce (& mut Deserializer < 'event , I >) -> Result < T , Error > > (& mut self , f : F ,) -> Result < T , Error > { let prev = mem :: replace (& mut self . in_plist_value , true) ; let ret = f (& mut * self) ; self . in_plist_value = prev ; ret } }
};
}
