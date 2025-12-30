// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
impl Widget for Example { fn render (self , area : Rect , buf : & mut Buffer) { let title_height = get_description_height (& self . description) ; let layout = Layout :: vertical ([Length (title_height) , Fill (0)]) ; let [title , illustrations] = area . layout (& layout) ; let (blocks , spacers) = Layout :: horizontal (& self . constraints) . flex (self . flex) . spacing (self . spacing) . split_with_spacers (illustrations) ; if ! self . description . is_empty () { Paragraph :: new (self . description . split ('\n') . map (| s | format ! ("// {s}") . italic () . fg (tailwind :: SLATE . c400)) . map (Line :: from) . collect :: < Vec < Line > > () ,) . render (title , buf) ; } for (block , constraint) in blocks . iter () . zip (& self . constraints) { Self :: illustration (* constraint , block . width) . render (* block , buf) ; } for spacer in spacers . iter () { Self :: render_spacer (* spacer , buf) ; } } }
};
}
