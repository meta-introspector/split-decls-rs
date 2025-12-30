// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () -> Result < () > { let mut message = std :: env :: current_dir () . unwrap () ; message . push ("message.png") ; let file = StorageFile :: GetFileFromPathAsync (& HSTRING :: from (message . to_str () . unwrap ())) ? . join () ? ; let stream = file . OpenAsync (FileAccessMode :: Read) ? . join () ? ; let decode = BitmapDecoder :: CreateAsync (& stream) ? . join () ? ; let bitmap = decode . GetSoftwareBitmapAsync () ? . join () ? ; let engine = OcrEngine :: TryCreateFromUserProfileLanguages () ? ; let result = engine . RecognizeAsync (& bitmap) ? . join () ? ; println ! ("{:?}" , result . Text () ?) ; Ok (()) }
};
}
