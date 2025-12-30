// Generated macro for impl_85 (impl)
macro_rules! Depcrate_hello_worldimpl_85 {
() => {
// Module: crate::hello_world
// Provides: {"impl_85"}
// Dependencies: {}
impl HelloWorldProvider { const DATA : & 'static [(& 'static str , & 'static str , & 'static str)] = & [("bn" , "" , "ওহে বিশ্ব") , ("cs" , "" , "Ahoj světe") , ("de" , "" , "Hallo Welt") , ("de" , "lowercase" , "hallo welt") , ("de" , "uppercase" , "HALLO WELT") , ("de-AT" , "" , "Servus Welt") , ("el" , "" , "Καλημέρα κόσμε") , ("en" , "" , "Hello World") , ("en-001" , "" , "Hello from 🗺️") , ("en-002" , "" , "Hello from 🌍") , ("en-019" , "" , "Hello from 🌎") , ("en-142" , "" , "Hello from 🌏") , ("en-GB" , "" , "Hello from 🇬🇧") , ("en-GB-u-sd-gbeng" , "" , "Hello from 🏴󠁧󠁢󠁥󠁮󠁧󠁿") , ("en" , "lowercase" , "hello world") , ("en" , "reverse" , "Olleh Dlrow") , ("en" , "rotate1" , "dHello Worl") , ("en" , "rotate2" , "ldHello Wor") , ("en" , "rotate3" , "rldHello Wo") , ("en" , "uppercase" , "HELLO WORLD") , ("eo" , "" , "Saluton, Mondo") , ("fa" , "" , "سلام دنیا‎") , ("fi" , "" , "hei maailma") , ("is" , "" , "Halló, heimur") , ("ja" , "" , "こんにちは世界") , ("ja" , "reverse" , "界世はちにんこ") , ("la" , "" , "Ave, munde") , ("pt" , "" , "Olá, mundo") , ("ro" , "" , "Salut, lume") , ("ru" , "" , "Привет, мир") , ("sr" , "" , "Поздрав свете") , ("sr-Latn" , "" , "Pozdrav svete") , ("vi" , "" , "Xin chào thế giới") , ("zh" , "" , "你好世界") ,] ; # [doc = " Converts this provider into a [`BufferProvider`] that uses JSON serialization."] # [cfg (feature = "deserialize_json")] pub fn into_json_provider (self) -> HelloWorldJsonProvider { HelloWorldJsonProvider } }
};
}
